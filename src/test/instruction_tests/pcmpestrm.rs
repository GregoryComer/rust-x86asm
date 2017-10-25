use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pcmpestrm_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPESTRM, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(104)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 96, 221, 104], OperandSize::Dword)
}

fn pcmpestrm_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPESTRM, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 308197250, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(92)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 96, 44, 221, 130, 183, 94, 18, 92], OperandSize::Dword)
}

fn pcmpestrm_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPESTRM, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(75)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 96, 233, 75], OperandSize::Qword)
}

fn pcmpestrm_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPESTRM, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(RCX, 1589365347, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(58)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 96, 153, 99, 202, 187, 94, 58], OperandSize::Qword)
}

