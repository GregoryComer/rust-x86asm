use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpslld_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(31)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 114, 245, 31], OperandSize::Dword)
}

#[test]
fn vpslld_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(80)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 114, 244, 80], OperandSize::Qword)
}

#[test]
fn vpslld_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(34)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 114, 244, 34], OperandSize::Dword)
}

#[test]
fn vpslld_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(24)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 114, 243, 24], OperandSize::Qword)
}

#[test]
fn vpslld_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(114)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 85, 141, 114, 244, 114], OperandSize::Dword)
}

#[test]
fn vpslld_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 217247095, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(83)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 85, 139, 114, 52, 77, 119, 237, 242, 12, 83], OperandSize::Dword)
}

#[test]
fn vpslld_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 1455701156, Some(OperandSize::Dword), None)), operand3: Some(Literal8(20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 101, 159, 114, 52, 77, 164, 60, 196, 86, 20], OperandSize::Dword)
}

#[test]
fn vpslld_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM17)), operand3: Some(Literal8(68)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 177, 101, 133, 114, 241, 68], OperandSize::Qword)
}

#[test]
fn vpslld_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM31)), operand2: Some(IndirectScaledIndexed(RDI, RBX, Four, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(46)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 5, 131, 114, 52, 159, 46], OperandSize::Qword)
}

#[test]
fn vpslld_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand3: Some(Literal8(46)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 85, 155, 114, 55, 46], OperandSize::Qword)
}

#[test]
fn vpslld_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Literal8(23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 117, 172, 114, 242, 23], OperandSize::Dword)
}

#[test]
fn vpslld_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM3)), operand2: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(67)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 101, 170, 114, 54, 67], OperandSize::Dword)
}

#[test]
fn vpslld_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 878969138, Some(OperandSize::Dword), None)), operand3: Some(Literal8(92)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 69, 190, 114, 52, 253, 50, 1, 100, 52, 92], OperandSize::Dword)
}

#[test]
fn vpslld_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM26)), operand3: Some(Literal8(55)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 145, 77, 169, 114, 242, 55], OperandSize::Qword)
}

#[test]
fn vpslld_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM15)), operand2: Some(IndirectDisplaced(RSI, 914070768, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 5, 171, 114, 182, 240, 156, 123, 54, 31], OperandSize::Qword)
}

#[test]
fn vpslld_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Four, 1883181185, Some(OperandSize::Dword), None)), operand3: Some(Literal8(38)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 101, 185, 114, 180, 190, 129, 16, 63, 112, 38], OperandSize::Qword)
}

#[test]
fn vpslld_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM3)), operand3: Some(Literal8(6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 202, 114, 243, 6], OperandSize::Dword)
}

#[test]
fn vpslld_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Four, 1879407247, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(78)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 85, 201, 114, 180, 144, 143, 122, 5, 112, 78], OperandSize::Dword)
}

#[test]
fn vpslld_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Four, 528923242, Some(OperandSize::Dword), None)), operand3: Some(Literal8(97)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 77, 222, 114, 180, 137, 106, 186, 134, 31, 97], OperandSize::Dword)
}

#[test]
fn vpslld_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM9)), operand3: Some(Literal8(98)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 209, 5, 197, 114, 241, 98], OperandSize::Qword)
}

#[test]
fn vpslld_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(ZMM20)), operand2: Some(IndirectScaledIndexed(RBX, RDI, Eight, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(37)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 93, 195, 114, 52, 251, 37], OperandSize::Qword)
}

#[test]
fn vpslld_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(ZMM15)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Two, 211297854, Some(OperandSize::Dword), None)), operand3: Some(Literal8(21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 5, 218, 114, 180, 112, 62, 38, 152, 12, 21], OperandSize::Qword)
}

#[test]
fn vpslld_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 242, 250], OperandSize::Dword)
}

#[test]
fn vpslld_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 242, 0], OperandSize::Dword)
}

#[test]
fn vpslld_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 242, 249], OperandSize::Qword)
}

#[test]
fn vpslld_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 242, 18], OperandSize::Qword)
}

#[test]
fn vpslld_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 242, 255], OperandSize::Dword)
}

#[test]
fn vpslld_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 242, 34], OperandSize::Dword)
}

#[test]
fn vpslld_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 242, 199], OperandSize::Qword)
}

#[test]
fn vpslld_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Eight, 434886147, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 242, 132, 242, 3, 214, 235, 25], OperandSize::Qword)
}

#[test]
fn vpslld_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 93, 141, 242, 245], OperandSize::Dword)
}

#[test]
fn vpslld_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 69, 138, 242, 34], OperandSize::Dword)
}

#[test]
fn vpslld_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM26)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 45, 131, 242, 246], OperandSize::Qword)
}

#[test]
fn vpslld_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 1322795836, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 101, 132, 242, 4, 205, 60, 67, 216, 78], OperandSize::Qword)
}

#[test]
fn vpslld_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 85, 174, 242, 226], OperandSize::Dword)
}

#[test]
fn vpslld_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(EBX, 1122454551, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 85, 174, 242, 139, 23, 76, 231, 66], OperandSize::Dword)
}

#[test]
fn vpslld_37() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM16)), operand3: Some(Direct(XMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 209, 125, 163, 242, 229], OperandSize::Qword)
}

#[test]
fn vpslld_38() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM8)), operand3: Some(IndirectDisplaced(RDX, 887336876, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 61, 173, 242, 130, 172, 175, 227, 52], OperandSize::Qword)
}

#[test]
fn vpslld_39() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 69, 201, 242, 209], OperandSize::Dword)
}

#[test]
fn vpslld_40() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(EDI, ESI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 69, 202, 242, 52, 247], OperandSize::Dword)
}

#[test]
fn vpslld_41() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 77, 201, 242, 205], OperandSize::Qword)
}

#[test]
fn vpslld_42() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM0)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 125, 207, 242, 22], OperandSize::Qword)
}

