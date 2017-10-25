use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn blsmsk_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSMSK, operand1: Some(Direct(EDX)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 104, 243, 211], OperandSize::Dword)
}

fn blsmsk_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSMSK, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexed(EBX, EBX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 112, 243, 20, 155], OperandSize::Dword)
}

fn blsmsk_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSMSK, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 88, 243, 213], OperandSize::Qword)
}

fn blsmsk_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSMSK, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 823060596, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 88, 243, 20, 85, 116, 232, 14, 49], OperandSize::Qword)
}

fn blsmsk_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSMSK, operand1: Some(Direct(RBP)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 208, 243, 210], OperandSize::Qword)
}

fn blsmsk_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSMSK, operand1: Some(Direct(RSI)), operand2: Some(IndirectDisplaced(RDI, 1779080715, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 200, 243, 151, 11, 158, 10, 106], OperandSize::Qword)
}

