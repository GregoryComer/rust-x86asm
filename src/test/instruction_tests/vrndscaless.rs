use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vrndscaless_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALESS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(22)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 93, 153, 10, 230, 22], OperandSize::Dword)
}

fn vrndscaless_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALESS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EAX, 1421076202, Some(OperandSize::Dword), None)), operand4: Some(Literal8(112)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 85, 137, 10, 152, 234, 230, 179, 84, 112], OperandSize::Dword)
}

fn vrndscaless_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALESS, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM18)), operand4: Some(Literal8(59)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 51, 101, 154, 10, 250, 59], OperandSize::Qword)
}

fn vrndscaless_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALESS, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM25)), operand3: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(122)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 99, 53, 131, 10, 32, 122], OperandSize::Qword)
}

