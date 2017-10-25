use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pmuldq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 40, 243], OperandSize::Dword)
}

fn pmuldq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULDQ, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 40, 31], OperandSize::Dword)
}

fn pmuldq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 40, 223], OperandSize::Qword)
}

fn pmuldq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULDQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(RBX, 186760586, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 40, 187, 138, 189, 33, 11], OperandSize::Qword)
}

