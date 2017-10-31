use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtpi2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 42, 192], OperandSize::Word)
}

#[test]
fn cvtpi2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 1153, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 42, 154, 129, 4], OperandSize::Word)
}

#[test]
fn cvtpi2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 42, 213], OperandSize::Dword)
}

#[test]
fn cvtpi2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(EDI, ECX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 42, 4, 207], OperandSize::Dword)
}

#[test]
fn cvtpi2pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 42, 240], OperandSize::Qword)
}

#[test]
fn cvtpi2pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PD, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 42, 56], OperandSize::Qword)
}

