use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmuludq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 244, 234], OperandSize::Dword)
}

#[test]
fn vpmuludq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 51759263, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 244, 28, 69, 159, 200, 21, 3], OperandSize::Dword)
}

#[test]
fn vpmuludq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 244, 215], OperandSize::Qword)
}

#[test]
fn vpmuludq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 2058039298, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 244, 52, 85, 2, 48, 171, 122], OperandSize::Qword)
}

#[test]
fn vpmuludq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 244, 250], OperandSize::Dword)
}

#[test]
fn vpmuludq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Two, 1498435807, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 244, 164, 79, 223, 80, 80, 89], OperandSize::Dword)
}

#[test]
fn vpmuludq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 244, 223], OperandSize::Qword)
}

#[test]
fn vpmuludq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(RAX, 390024673, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 244, 128, 225, 77, 63, 23], OperandSize::Qword)
}

#[test]
fn vpmuludq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 213, 143, 244, 192], OperandSize::Dword)
}

#[test]
fn vpmuludq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Eight, 916470569, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 197, 139, 244, 172, 195, 41, 59, 160, 54], OperandSize::Dword)
}

#[test]
fn vpmuludq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(EDI, ECX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 237, 155, 244, 60, 79], OperandSize::Dword)
}

#[test]
fn vpmuludq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 129, 237, 138, 244, 208], OperandSize::Qword)
}

#[test]
fn vpmuludq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectDisplaced(RDX, 1541713569, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 229, 133, 244, 130, 161, 174, 228, 91], OperandSize::Qword)
}

#[test]
fn vpmuludq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM17)), operand3: Some(IndirectDisplaced(RBX, 1086787477, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 245, 149, 244, 171, 149, 15, 199, 64], OperandSize::Qword)
}

#[test]
fn vpmuludq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 237, 171, 244, 235], OperandSize::Dword)
}

#[test]
fn vpmuludq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(ECX, 1048934740, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 213, 174, 244, 169, 84, 121, 133, 62], OperandSize::Dword)
}

#[test]
fn vpmuludq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 175688454, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 237, 185, 244, 52, 93, 6, 203, 120, 10], OperandSize::Dword)
}

#[test]
fn vpmuludq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 129, 213, 171, 244, 231], OperandSize::Qword)
}

#[test]
fn vpmuludq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM9)), operand3: Some(IndirectDisplaced(RBX, 272603885, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 181, 173, 244, 139, 237, 154, 63, 16], OperandSize::Qword)
}

#[test]
fn vpmuludq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM13)), operand3: Some(IndirectDisplaced(RDI, 1662353289, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 113, 149, 191, 244, 167, 137, 127, 21, 99], OperandSize::Qword)
}

#[test]
fn vpmuludq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 213, 207, 244, 201], OperandSize::Dword)
}

#[test]
fn vpmuludq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Eight, 316595537, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 205, 205, 244, 164, 215, 81, 221, 222, 18], OperandSize::Dword)
}

#[test]
fn vpmuludq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Two, 1027385473, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 205, 221, 244, 132, 119, 129, 168, 60, 61], OperandSize::Dword)
}

#[test]
fn vpmuludq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM10)), operand3: Some(Direct(ZMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 81, 173, 206, 244, 222], OperandSize::Qword)
}

#[test]
fn vpmuludq_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM19)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Eight, 1426393677, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 229, 194, 244, 172, 211, 77, 10, 5, 85], OperandSize::Qword)
}

#[test]
fn vpmuludq_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Eight, 701925925, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 113, 245, 221, 244, 156, 218, 37, 138, 214, 41], OperandSize::Qword)
}

