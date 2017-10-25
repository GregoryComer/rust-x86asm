use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn movdqu_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQU, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 111, 193], OperandSize::Dword)
}

fn movdqu_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQU, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Eight, 770219967, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 111, 148, 192, 191, 159, 232, 45], OperandSize::Dword)
}

fn movdqu_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQU, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 111, 244], OperandSize::Qword)
}

fn movdqu_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQU, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Four, 931392711, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 111, 172, 154, 199, 236, 131, 55], OperandSize::Qword)
}

fn movdqu_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQU, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 111, 209], OperandSize::Dword)
}

fn movdqu_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQU, operand1: Some(IndirectDisplaced(EDX, 1131359282, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 127, 146, 50, 44, 111, 67], OperandSize::Dword)
}

fn movdqu_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQU, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 111, 233], OperandSize::Qword)
}

fn movdqu_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDQU, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Four, 1245041918, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 127, 164, 177, 254, 212, 53, 74], OperandSize::Qword)
}

