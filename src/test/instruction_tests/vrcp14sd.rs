use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vrcp14sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 197, 141, 77, 235], OperandSize::Dword)
}

fn vrcp14sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 237, 138, 77, 51], OperandSize::Dword)
}

fn vrcp14sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM18)), operand3: Some(Direct(XMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 210, 237, 135, 77, 227], OperandSize::Qword)
}

fn vrcp14sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14SD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectDisplaced(RDI, 670231149, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 165, 130, 77, 143, 109, 234, 242, 39], OperandSize::Qword)
}

