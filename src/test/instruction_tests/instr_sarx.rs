use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sarx_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SARX, operand1: Some(Direct(EDX)), operand2: Some(Direct(ECX)), operand3: Some(Direct(EBP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 82, 247, 209], OperandSize::Dword)
}

#[test]
fn sarx_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SARX, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 1094926233, Some(OperandSize::Dword), None)), operand3: Some(Direct(EDI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 66, 247, 20, 197, 153, 63, 67, 65], OperandSize::Dword)
}

#[test]
fn sarx_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SARX, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBX)), operand3: Some(Direct(EDX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 106, 247, 243], OperandSize::Qword)
}

#[test]
fn sarx_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SARX, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Two, 1479521235, Some(OperandSize::Dword), None)), operand3: Some(Direct(EDX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 106, 247, 164, 115, 211, 179, 47, 88], OperandSize::Qword)
}

#[test]
fn sarx_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SARX, operand1: Some(Direct(RSP)), operand2: Some(Direct(RDX)), operand3: Some(Direct(RDX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 234, 247, 226], OperandSize::Qword)
}

#[test]
fn sarx_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SARX, operand1: Some(Direct(RDX)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Four, 1746303393, Some(OperandSize::Qword), None)), operand3: Some(Direct(RSP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 218, 247, 148, 150, 161, 121, 22, 104], OperandSize::Qword)
}

