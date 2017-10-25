use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpminuq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 245, 142, 59, 221], OperandSize::Dword)
}

#[test]
fn vpminuq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EDI, ECX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 138, 59, 12, 207], OperandSize::Dword)
}

#[test]
fn vpminuq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(EBX, 341840863, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 155, 59, 163, 223, 19, 96, 20], OperandSize::Dword)
}

#[test]
fn vpminuq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM31)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 133, 131, 59, 226], OperandSize::Qword)
}

#[test]
fn vpminuq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM17)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 524047520, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 245, 132, 59, 20, 197, 160, 84, 60, 31], OperandSize::Qword)
}

#[test]
fn vpminuq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Two, 632715537, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 237, 153, 59, 156, 81, 17, 121, 182, 37], OperandSize::Qword)
}

#[test]
fn vpminuq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 205, 172, 59, 200], OperandSize::Dword)
}

#[test]
fn vpminuq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EAX, EAX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 170, 59, 4, 64], OperandSize::Dword)
}

#[test]
fn vpminuq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(EDX, ESI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 245, 186, 59, 36, 242], OperandSize::Dword)
}

#[test]
fn vpminuq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM13)), operand3: Some(Direct(YMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 194, 149, 175, 59, 200], OperandSize::Qword)
}

#[test]
fn vpminuq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM12)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 1685985697, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 157, 171, 59, 20, 245, 161, 25, 126, 100], OperandSize::Qword)
}

#[test]
fn vpminuq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM29)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1474719744, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 149, 177, 59, 52, 117, 0, 112, 230, 87], OperandSize::Qword)
}

#[test]
fn vpminuq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 221, 205, 59, 249], OperandSize::Dword)
}

#[test]
fn vpminuq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: Some(Indirect(EAX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 204, 59, 8], OperandSize::Dword)
}

#[test]
fn vpminuq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectDisplaced(EAX, 747599334, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 229, 219, 59, 152, 230, 117, 143, 44], OperandSize::Dword)
}

#[test]
fn vpminuq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM30)), operand3: Some(Direct(ZMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 66, 141, 197, 59, 243], OperandSize::Qword)
}

#[test]
fn vpminuq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM21)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Two, 41449549, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 213, 193, 59, 156, 64, 77, 120, 120, 2], OperandSize::Qword)
}

#[test]
fn vpminuq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM0)), operand3: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 253, 218, 59, 7], OperandSize::Qword)
}

