use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmovmskb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVMSKB, operand1: Some(Direct(EDI)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 215, 253], OperandSize::Dword)
}

fn vpmovmskb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVMSKB, operand1: Some(Direct(RBX)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 215, 219], OperandSize::Qword)
}

fn vpmovmskb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVMSKB, operand1: Some(Direct(EBX)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 215, 221], OperandSize::Dword)
}

fn vpmovmskb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVMSKB, operand1: Some(Direct(RDI)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 215, 253], OperandSize::Qword)
}

