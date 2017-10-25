use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pmovmskb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVMSKB, operand1: Some(Direct(ESP)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 215, 230], OperandSize::Dword)
}

fn pmovmskb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVMSKB, operand1: Some(Direct(RBP)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 215, 238], OperandSize::Qword)
}

fn pmovmskb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVMSKB, operand1: Some(Direct(EBX)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 215, 219], OperandSize::Dword)
}

fn pmovmskb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVMSKB, operand1: Some(Direct(RBP)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 215, 237], OperandSize::Qword)
}

