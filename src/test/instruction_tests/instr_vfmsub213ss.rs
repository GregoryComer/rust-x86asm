use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsub213ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 171, 246], OperandSize::Dword)
}

#[test]
fn vfmsub213ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Eight, 1922536898, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 171, 188, 251, 194, 149, 151, 114], OperandSize::Dword)
}

#[test]
fn vfmsub213ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 171, 225], OperandSize::Qword)
}

#[test]
fn vfmsub213ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RDX, RAX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 171, 44, 130], OperandSize::Qword)
}

#[test]
fn vfmsub213ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 189, 171, 241], OperandSize::Dword)
}

#[test]
fn vfmsub213ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Eight, 1701689530, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 137, 171, 148, 241, 186, 184, 109, 101], OperandSize::Dword)
}

#[test]
fn vfmsub213ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SS, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM25)), operand3: Some(Direct(XMM8)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 194, 53, 241, 171, 248], OperandSize::Qword)
}

#[test]
fn vfmsub213ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SS, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM30)), operand3: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 13, 130, 171, 30], OperandSize::Qword)
}

