use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vexp2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 157, 200, 193], OperandSize::Dword)
}

#[test]
fn vexp2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PS, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectDisplaced(EAX, 849965455, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 203, 200, 152, 143, 113, 169, 50], OperandSize::Dword)
}

#[test]
fn vexp2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PS, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 1847621743, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 221, 200, 36, 253, 111, 120, 32, 110], OperandSize::Dword)
}

#[test]
fn vexp2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PS, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 66, 125, 159, 200, 223], OperandSize::Qword)
}

#[test]
fn vexp2ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PS, operand1: Some(Direct(ZMM27)), operand2: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 125, 201, 200, 24], OperandSize::Qword)
}

#[test]
fn vexp2ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PS, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Eight, 2125057762, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 221, 200, 132, 208, 226, 206, 169, 126], OperandSize::Qword)
}

