use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cvtsi2sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 42, 239], OperandSize::Dword)
}

fn cvtsi2sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 1138343242, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 42, 36, 149, 74, 189, 217, 67], OperandSize::Dword)
}

fn cvtsi2sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 42, 228], OperandSize::Qword)
}

fn cvtsi2sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 2017877995, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 42, 4, 205, 235, 95, 70, 120], OperandSize::Qword)
}

fn cvtsi2sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 72, 15, 42, 228], OperandSize::Qword)
}

fn cvtsi2sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(RCX, 376213893, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 72, 15, 42, 177, 133, 145, 108, 22], OperandSize::Qword)
}

