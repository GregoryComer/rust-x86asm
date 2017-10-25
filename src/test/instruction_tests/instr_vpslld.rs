use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpslld_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 114, 241, 2], OperandSize::Dword)
}

#[test]
fn vpslld_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(117)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 114, 242, 117], OperandSize::Qword)
}

#[test]
fn vpslld_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(54)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 114, 240, 54], OperandSize::Dword)
}

#[test]
fn vpslld_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Literal8(119)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 114, 241, 119], OperandSize::Qword)
}

#[test]
fn vpslld_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(107)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 138, 114, 247, 107], OperandSize::Dword)
}

#[test]
fn vpslld_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Two, 802412003, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(96)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 85, 140, 114, 180, 95, 227, 213, 211, 47, 96], OperandSize::Dword)
}

#[test]
fn vpslld_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Eight, 1579492856, Some(OperandSize::Dword), None)), operand3: Some(Literal8(76)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 101, 159, 114, 180, 203, 248, 37, 37, 94, 76], OperandSize::Dword)
}

#[test]
fn vpslld_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM9)), operand3: Some(Literal8(11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 209, 93, 129, 114, 241, 11], OperandSize::Qword)
}

#[test]
fn vpslld_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM11)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(69)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 37, 139, 114, 49, 69], OperandSize::Qword)
}

#[test]
fn vpslld_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM30)), operand2: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(46)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 13, 145, 114, 48, 46], OperandSize::Qword)
}

#[test]
fn vpslld_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(120)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 77, 173, 114, 247, 120], OperandSize::Dword)
}

#[test]
fn vpslld_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Two, 1147985678, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(122)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 171, 114, 180, 115, 14, 223, 108, 68, 122], OperandSize::Dword)
}

#[test]
fn vpslld_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 1046238311, Some(OperandSize::Dword), None)), operand3: Some(Literal8(44)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 117, 190, 114, 52, 245, 103, 84, 92, 62, 44], OperandSize::Dword)
}

#[test]
fn vpslld_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM23)), operand3: Some(Literal8(40)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 177, 93, 163, 114, 247, 40], OperandSize::Qword)
}

#[test]
fn vpslld_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM24)), operand2: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(71)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 61, 164, 114, 55, 71], OperandSize::Qword)
}

#[test]
fn vpslld_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexed(RDI, RDX, Eight, Some(OperandSize::Dword), None)), operand3: Some(Literal8(71)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 93, 187, 114, 52, 215, 71], OperandSize::Qword)
}

#[test]
fn vpslld_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM6)), operand3: Some(Literal8(95)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 85, 202, 114, 246, 95], OperandSize::Dword)
}

#[test]
fn vpslld_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 465388860, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(120)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 205, 114, 52, 157, 60, 69, 189, 27, 120], OperandSize::Dword)
}

#[test]
fn vpslld_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Eight, 119321759, Some(OperandSize::Dword), None)), operand3: Some(Literal8(60)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 85, 218, 114, 180, 200, 159, 180, 28, 7, 60], OperandSize::Dword)
}

#[test]
fn vpslld_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM3)), operand3: Some(Literal8(75)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 29, 202, 114, 243, 75], OperandSize::Qword)
}

#[test]
fn vpslld_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(ZMM21)), operand2: Some(IndirectDisplaced(RAX, 801872097, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(111)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 85, 196, 114, 176, 225, 152, 203, 47, 111], OperandSize::Qword)
}

#[test]
fn vpslld_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(ZMM11)), operand2: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 37, 222, 114, 51, 12], OperandSize::Qword)
}

#[test]
fn vpslld_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 242, 248], OperandSize::Dword)
}

#[test]
fn vpslld_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(ECX, 981017854, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 242, 161, 254, 36, 121, 58], OperandSize::Dword)
}

#[test]
fn vpslld_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 242, 205], OperandSize::Qword)
}

#[test]
fn vpslld_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 2128561721, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 242, 60, 117, 57, 70, 223, 126], OperandSize::Qword)
}

#[test]
fn vpslld_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 242, 201], OperandSize::Dword)
}

#[test]
fn vpslld_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 242, 49], OperandSize::Dword)
}

#[test]
fn vpslld_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 242, 248], OperandSize::Qword)
}

#[test]
fn vpslld_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(RAX, 1001720894, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 242, 128, 62, 12, 181, 59], OperandSize::Qword)
}

#[test]
fn vpslld_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 77, 143, 242, 197], OperandSize::Dword)
}

#[test]
fn vpslld_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EBX, 2146526572, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 101, 143, 242, 155, 108, 101, 241, 127], OperandSize::Dword)
}

#[test]
fn vpslld_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 1, 29, 142, 242, 240], OperandSize::Qword)
}

#[test]
fn vpslld_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Eight, 1711183372, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 101, 137, 242, 164, 206, 12, 150, 254, 101], OperandSize::Qword)
}

#[test]
fn vpslld_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 109, 171, 242, 243], OperandSize::Dword)
}

#[test]
fn vpslld_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 171, 242, 49], OperandSize::Dword)
}

#[test]
fn vpslld_37() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM25)), operand3: Some(Direct(XMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 129, 53, 164, 242, 218], OperandSize::Qword)
}

#[test]
fn vpslld_38() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM14)), operand3: Some(IndirectScaledIndexed(RDI, RCX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 13, 172, 242, 44, 143], OperandSize::Qword)
}

#[test]
fn vpslld_39() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 101, 206, 242, 224], OperandSize::Dword)
}

#[test]
fn vpslld_40() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Two, 974118054, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 77, 203, 242, 132, 90, 166, 220, 15, 58], OperandSize::Dword)
}

#[test]
fn vpslld_41() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM11)), operand3: Some(Direct(XMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 33, 37, 207, 242, 239], OperandSize::Qword)
}

#[test]
fn vpslld_42() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Two, 1436531945, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 93, 206, 242, 188, 70, 233, 188, 159, 85], OperandSize::Qword)
}

