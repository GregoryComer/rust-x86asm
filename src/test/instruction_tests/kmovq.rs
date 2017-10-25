use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn kmovq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVQ, operand1: Some(Direct(K4)), operand2: Some(Direct(K4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 248, 144, 228], OperandSize::Dword)
}

fn kmovq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVQ, operand1: Some(Direct(K6)), operand2: Some(IndirectScaledIndexed(EAX, ECX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 248, 144, 52, 136], OperandSize::Dword)
}

fn kmovq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVQ, operand1: Some(Direct(K7)), operand2: Some(Direct(K5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 248, 144, 253], OperandSize::Qword)
}

fn kmovq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVQ, operand1: Some(Direct(K7)), operand2: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 248, 144, 58], OperandSize::Qword)
}

fn kmovq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVQ, operand1: Some(IndirectScaledIndexed(ECX, ESI, Eight, Some(OperandSize::Qword), None)), operand2: Some(Direct(K4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 248, 145, 36, 241], OperandSize::Dword)
}

fn kmovq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVQ, operand1: Some(IndirectScaledIndexed(RDX, RCX, Four, Some(OperandSize::Qword), None)), operand2: Some(Direct(K2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 248, 145, 20, 138], OperandSize::Qword)
}

fn kmovq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVQ, operand1: Some(Direct(K4)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 251, 146, 230], OperandSize::Qword)
}

fn kmovq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVQ, operand1: Some(Direct(RBX)), operand2: Some(Direct(K3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 251, 147, 219], OperandSize::Qword)
}

