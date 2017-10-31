use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmovzxbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 48, 228], OperandSize::Dword)
}

#[test]
fn pmovzxbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(ECX, 1040626112, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 48, 185, 192, 177, 6, 62], OperandSize::Dword)
}

#[test]
fn pmovzxbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 48, 218], OperandSize::Qword)
}

#[test]
fn pmovzxbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(RSI, RDX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 48, 4, 150], OperandSize::Qword)
}

