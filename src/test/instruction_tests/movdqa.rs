use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn movdqa_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQA, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 111, 218], OperandSize::Dword)
}

fn movdqa_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQA, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(EBX, 420121985, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 111, 131, 129, 141, 10, 25], OperandSize::Dword)
}

fn movdqa_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQA, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 111, 254], OperandSize::Qword)
}

fn movdqa_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQA, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(RSI, Four, 895496629, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 111, 44, 181, 181, 49, 96, 53], OperandSize::Qword)
}

fn movdqa_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQA, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 111, 234], OperandSize::Dword)
}

fn movdqa_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQA, operand1: Some(IndirectScaledIndexed(EAX, ECX, Eight, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 127, 60, 200], OperandSize::Dword)
}

fn movdqa_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQA, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 111, 192], OperandSize::Qword)
}

fn movdqa_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQA, operand1: Some(IndirectDisplaced(RDI, 384692892, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 127, 183, 156, 242, 237, 22], OperandSize::Qword)
}

