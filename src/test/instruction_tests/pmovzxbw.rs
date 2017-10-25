use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pmovzxbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 48, 204], OperandSize::Dword)
}

fn pmovzxbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBW, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 48, 63], OperandSize::Dword)
}

fn pmovzxbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 48, 248], OperandSize::Qword)
}

fn pmovzxbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBW, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 48, 40], OperandSize::Qword)
}

