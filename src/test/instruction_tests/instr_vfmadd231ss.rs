use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmadd231ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 185, 192], OperandSize::Dword)
}

#[test]
fn vfmadd231ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 2104492460, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 185, 44, 157, 172, 1, 112, 125], OperandSize::Dword)
}

#[test]
fn vfmadd231ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 185, 234], OperandSize::Qword)
}

#[test]
fn vfmadd231ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 185, 17], OperandSize::Qword)
}

#[test]
fn vfmadd231ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 77, 187, 185, 199], OperandSize::Dword)
}

#[test]
fn vfmadd231ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Two, 355013495, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 101, 138, 185, 156, 78, 119, 19, 41, 21], OperandSize::Dword)
}

#[test]
fn vfmadd231ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SS, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 162, 29, 223, 185, 210], OperandSize::Qword)
}

#[test]
fn vfmadd231ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 21, 142, 185, 28, 146], OperandSize::Qword)
}

