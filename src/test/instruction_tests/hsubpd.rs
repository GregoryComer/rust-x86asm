use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn hsubpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::HSUBPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 125, 253], OperandSize::Dword)
}

fn hsubpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::HSUBPD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 728443337, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 125, 36, 93, 201, 41, 107, 43], OperandSize::Dword)
}

fn hsubpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::HSUBPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 125, 232], OperandSize::Qword)
}

fn hsubpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::HSUBPD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RDX, 278967665, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 125, 138, 113, 181, 160, 16], OperandSize::Qword)
}

