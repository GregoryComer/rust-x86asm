use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrcp28ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 101, 155, 203, 254], OperandSize::Dword)
}

#[test]
fn vrcp28ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Two, 329172286, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 69, 143, 203, 148, 119, 62, 197, 158, 19], OperandSize::Dword)
}

#[test]
fn vrcp28ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28SS, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 82, 29, 146, 203, 228], OperandSize::Qword)
}

#[test]
fn vrcp28ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28SS, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 101, 140, 203, 19], OperandSize::Qword)
}

