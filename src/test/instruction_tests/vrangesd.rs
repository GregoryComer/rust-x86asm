use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vrangesd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGESD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(91)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 229, 154, 81, 238, 91], OperandSize::Dword)
}

fn vrangesd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGESD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(101)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 205, 140, 81, 19, 101], OperandSize::Dword)
}

fn vrangesd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGESD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM20)), operand3: Some(Direct(XMM22)), operand4: Some(Literal8(56)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 35, 221, 146, 81, 206, 56], OperandSize::Qword)
}

fn vrangesd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGESD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Four, 342318581, Some(OperandSize::Qword), None)), operand4: Some(Literal8(39)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 99, 245, 141, 81, 140, 187, 245, 93, 103, 20, 39], OperandSize::Qword)
}

