use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtsi2sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 42, 222], OperandSize::Dword)
}

#[test]
fn cvtsi2sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(EDI, ECX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 42, 36, 79], OperandSize::Dword)
}

#[test]
fn cvtsi2sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 42, 238], OperandSize::Qword)
}

#[test]
fn cvtsi2sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 214607168, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 42, 44, 117, 64, 165, 202, 12], OperandSize::Qword)
}

#[test]
fn cvtsi2sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 72, 15, 42, 196], OperandSize::Qword)
}

#[test]
fn cvtsi2sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(RBX, 1936153762, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 72, 15, 42, 163, 162, 92, 103, 115], OperandSize::Qword)
}

