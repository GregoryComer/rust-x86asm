use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpminsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 57, 222], OperandSize::Dword)
}

#[test]
fn vpminsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(EBX, ECX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 57, 44, 75], OperandSize::Dword)
}

#[test]
fn vpminsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 57, 250], OperandSize::Qword)
}

#[test]
fn vpminsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(RAX, RDI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 57, 44, 248], OperandSize::Qword)
}

#[test]
fn vpminsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 57, 240], OperandSize::Dword)
}

#[test]
fn vpminsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(EAX, 64269720, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 57, 176, 152, 173, 212, 3], OperandSize::Dword)
}

#[test]
fn vpminsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 57, 232], OperandSize::Qword)
}

#[test]
fn vpminsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 913837202, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 57, 36, 117, 146, 12, 120, 54], OperandSize::Qword)
}

#[test]
fn vpminsd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 69, 142, 57, 218], OperandSize::Dword)
}

#[test]
fn vpminsd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(EDI, ESI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 109, 141, 57, 12, 247], OperandSize::Dword)
}

#[test]
fn vpminsd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 77, 155, 57, 0], OperandSize::Dword)
}

#[test]
fn vpminsd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 162, 109, 142, 57, 211], OperandSize::Qword)
}

#[test]
fn vpminsd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM17)), operand3: Some(IndirectDisplaced(RSI, 737822404, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 117, 131, 57, 134, 196, 70, 250, 43], OperandSize::Qword)
}

#[test]
fn vpminsd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM18)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 1654375177, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 109, 146, 57, 12, 181, 9, 195, 155, 98], OperandSize::Qword)
}

#[test]
fn vpminsd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 109, 173, 57, 250], OperandSize::Dword)
}

#[test]
fn vpminsd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 77, 175, 57, 58], OperandSize::Dword)
}

#[test]
fn vpminsd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Eight, 2061172196, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 190, 57, 172, 241, 228, 253, 218, 122], OperandSize::Dword)
}

#[test]
fn vpminsd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 162, 85, 171, 57, 237], OperandSize::Qword)
}

#[test]
fn vpminsd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM9)), operand3: Some(IndirectScaledIndexed(RBX, RBX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 53, 169, 57, 52, 219], OperandSize::Qword)
}

#[test]
fn vpminsd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Four, 948890884, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 125, 190, 57, 188, 144, 4, 237, 142, 56], OperandSize::Qword)
}

#[test]
fn vpminsd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 69, 202, 57, 200], OperandSize::Dword)
}

#[test]
fn vpminsd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Eight, 1292145950, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 85, 205, 57, 172, 209, 30, 149, 4, 77], OperandSize::Dword)
}

#[test]
fn vpminsd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 849055328, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 77, 217, 57, 60, 149, 96, 142, 155, 50], OperandSize::Dword)
}

#[test]
fn vpminsd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 162, 77, 202, 57, 223], OperandSize::Qword)
}

#[test]
fn vpminsd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Two, 1133222282, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 77, 201, 57, 188, 90, 138, 153, 139, 67], OperandSize::Qword)
}

#[test]
fn vpminsd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM25)), operand3: Some(IndirectScaledIndexed(RCX, RDI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 53, 213, 57, 28, 121], OperandSize::Qword)
}

