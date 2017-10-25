use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vgetmantss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM3)), operand4: Some(Literal8(26)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 85, 156, 39, 211, 26], OperandSize::Dword)
}

fn vgetmantss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(EBX, 1229022424, Some(OperandSize::Dword), None)), operand4: Some(Literal8(12)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 109, 142, 39, 163, 216, 100, 65, 73, 12], OperandSize::Dword)
}

fn vgetmantss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTSS, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM19)), operand3: Some(Direct(XMM19)), operand4: Some(Literal8(52)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 163, 101, 147, 39, 235, 52], OperandSize::Qword)
}

fn vgetmantss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTSS, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand4: Some(Literal8(49)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 227, 117, 143, 39, 55, 49], OperandSize::Qword)
}

