use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtpi2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 42, 214], OperandSize::Word)
}

#[test]
fn cvtpi2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PD, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(SI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 42, 44], OperandSize::Word)
}

#[test]
fn cvtpi2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 42, 228], OperandSize::Dword)
}

#[test]
fn cvtpi2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 1124251680, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 42, 36, 205, 32, 184, 2, 67], OperandSize::Dword)
}

#[test]
fn cvtpi2pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 42, 228], OperandSize::Qword)
}

#[test]
fn cvtpi2pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PD, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 42, 24], OperandSize::Qword)
}

