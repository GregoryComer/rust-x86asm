use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pcmpeqb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQB, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 116, 208], OperandSize::Dword)
}

fn pcmpeqb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQB, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Four, 59983481, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 116, 188, 153, 121, 70, 147, 3], OperandSize::Dword)
}

fn pcmpeqb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQB, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 116, 222], OperandSize::Qword)
}

fn pcmpeqb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQB, operand1: Some(Direct(MM2)), operand2: Some(IndirectDisplaced(RDX, 278069993, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 116, 146, 233, 2, 147, 16], OperandSize::Qword)
}

fn pcmpeqb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 116, 217], OperandSize::Dword)
}

fn pcmpeqb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQB, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 665222712, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 116, 20, 69, 56, 126, 166, 39], OperandSize::Dword)
}

fn pcmpeqb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 116, 206], OperandSize::Qword)
}

fn pcmpeqb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQB, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(RAX, 1032695026, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 116, 152, 242, 172, 141, 61], OperandSize::Qword)
}

