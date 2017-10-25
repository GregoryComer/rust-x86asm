use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pmovzxbq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 50, 219], OperandSize::Dword)
}

fn pmovzxbq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 2101592169, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 50, 28, 117, 105, 192, 67, 125], OperandSize::Dword)
}

fn pmovzxbq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 50, 215], OperandSize::Qword)
}

fn pmovzxbq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBQ, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(RDX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 50, 10], OperandSize::Qword)
}

