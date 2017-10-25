use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn movddup_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDDUP, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 18, 247], OperandSize::Dword)
}

fn movddup_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDDUP, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 18, 56], OperandSize::Dword)
}

fn movddup_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDDUP, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 18, 254], OperandSize::Qword)
}

fn movddup_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDDUP, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(RAX, 1764602209, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 18, 152, 97, 177, 45, 105], OperandSize::Qword)
}

