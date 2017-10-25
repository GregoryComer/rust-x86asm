use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pmovzxdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 53, 254], OperandSize::Dword)
}

fn pmovzxdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXDQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(EAX, 1593486788, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 53, 136, 196, 173, 250, 94], OperandSize::Dword)
}

fn pmovzxdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 53, 225], OperandSize::Qword)
}

fn pmovzxdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXDQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(RCX, 1299278892, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 53, 153, 44, 108, 113, 77], OperandSize::Qword)
}

