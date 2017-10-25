use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pcmpistrm_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPISTRM, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(118)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 98, 244, 118], OperandSize::Dword)
}

fn pcmpistrm_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPISTRM, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(ESI, EDI, Four, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(61)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 98, 4, 190, 61], OperandSize::Dword)
}

fn pcmpistrm_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPISTRM, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(120)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 98, 225, 120], OperandSize::Qword)
}

fn pcmpistrm_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPISTRM, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(RDX, 72981540, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(56)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 98, 130, 36, 156, 89, 4, 56], OperandSize::Qword)
}

