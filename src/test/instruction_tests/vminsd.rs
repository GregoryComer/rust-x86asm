use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vminsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 235, 93, 218], OperandSize::Dword)
}

fn vminsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EDI, ECX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 93, 52, 79], OperandSize::Dword)
}

fn vminsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 211, 93, 205], OperandSize::Qword)
}

fn vminsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RDX, 1737703073, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 243, 93, 138, 161, 62, 147, 103], OperandSize::Qword)
}

fn vminsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 231, 155, 93, 239], OperandSize::Dword)
}

fn vminsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 207, 137, 93, 2], OperandSize::Dword)
}

fn vminsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(XMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 209, 191, 153, 93, 244], OperandSize::Qword)
}

fn vminsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectScaledIndexed(RSI, RAX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 175, 137, 93, 12, 70], OperandSize::Qword)
}

