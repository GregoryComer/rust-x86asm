use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vminss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 194, 93, 228], OperandSize::Dword)
}

fn vminss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 202, 93, 34], OperandSize::Dword)
}

fn vminss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 218, 93, 202], OperandSize::Qword)
}

fn vminss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RDI, RCX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 210, 93, 28, 79], OperandSize::Qword)
}

fn vminss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 78, 153, 93, 235], OperandSize::Dword)
}

fn vminss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EDI, EBX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 86, 143, 93, 44, 159], OperandSize::Dword)
}

fn vminss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM27)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 38, 151, 93, 241], OperandSize::Qword)
}

fn vminss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM25)), operand3: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 54, 130, 93, 6], OperandSize::Qword)
}

