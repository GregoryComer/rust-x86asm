use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrcp28sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 237, 154, 203, 193], OperandSize::Dword)
}

#[test]
fn vrcp28sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(ECX, 766460007, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 139, 203, 145, 103, 64, 175, 45], OperandSize::Dword)
}

#[test]
fn vrcp28sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 178, 205, 157, 203, 241], OperandSize::Qword)
}

#[test]
fn vrcp28sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Four, 787085369, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 173, 129, 203, 188, 186, 57, 248, 233, 46], OperandSize::Qword)
}

