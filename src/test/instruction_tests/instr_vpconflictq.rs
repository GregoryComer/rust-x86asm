use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpconflictq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 142, 196, 243], OperandSize::Dword)
}

#[test]
fn vpconflictq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 2039316879, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 138, 196, 36, 69, 143, 129, 141, 121], OperandSize::Dword)
}

#[test]
fn vpconflictq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 154, 196, 49], OperandSize::Dword)
}

#[test]
fn vpconflictq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 162, 253, 142, 196, 240], OperandSize::Qword)
}

#[test]
fn vpconflictq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(XMM16)), operand2: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 253, 139, 196, 2], OperandSize::Qword)
}

#[test]
fn vpconflictq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(XMM28)), operand2: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 98, 253, 153, 196, 38], OperandSize::Qword)
}

#[test]
fn vpconflictq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 169, 196, 246], OperandSize::Dword)
}

#[test]
fn vpconflictq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(YMM4)), operand2: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 172, 196, 39], OperandSize::Dword)
}

#[test]
fn vpconflictq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(YMM2)), operand2: Some(IndirectDisplaced(EBX, 1885106558, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 185, 196, 147, 126, 113, 92, 112], OperandSize::Dword)
}

#[test]
fn vpconflictq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 194, 253, 172, 196, 237], OperandSize::Qword)
}

#[test]
fn vpconflictq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(YMM13)), operand2: Some(IndirectScaledIndexed(RCX, RSI, Four, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 253, 172, 196, 44, 177], OperandSize::Qword)
}

#[test]
fn vpconflictq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 1921299318, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 187, 196, 36, 77, 118, 179, 132, 114], OperandSize::Qword)
}

#[test]
fn vpconflictq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 203, 196, 251], OperandSize::Dword)
}

#[test]
fn vpconflictq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Two, 1233427830, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 205, 196, 188, 122, 118, 157, 132, 73], OperandSize::Dword)
}

#[test]
fn vpconflictq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 1089063134, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 219, 196, 60, 93, 222, 200, 233, 64], OperandSize::Dword)
}

#[test]
fn vpconflictq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 194, 253, 201, 196, 236], OperandSize::Qword)
}

#[test]
fn vpconflictq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(ZMM17)), operand2: Some(IndirectDisplaced(RDX, 831961600, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 253, 206, 196, 138, 0, 186, 150, 49], OperandSize::Qword)
}

#[test]
fn vpconflictq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(ZMM11)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Two, 1368626799, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 253, 221, 196, 156, 114, 111, 150, 147, 81], OperandSize::Qword)
}

