use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pcmpeqw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQW, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 117, 195], OperandSize::Dword)
}

fn pcmpeqw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQW, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledIndexed(ESI, ECX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 117, 60, 206], OperandSize::Dword)
}

fn pcmpeqw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQW, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 117, 217], OperandSize::Qword)
}

fn pcmpeqw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQW, operand1: Some(Direct(MM6)), operand2: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 117, 51], OperandSize::Qword)
}

fn pcmpeqw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 117, 254], OperandSize::Dword)
}

fn pcmpeqw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(EBX, EAX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 117, 52, 67], OperandSize::Dword)
}

fn pcmpeqw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 117, 234], OperandSize::Qword)
}

fn pcmpeqw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQW, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Four, 1531556393, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 117, 172, 154, 41, 178, 73, 91], OperandSize::Qword)
}

