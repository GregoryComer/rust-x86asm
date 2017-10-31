use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmadd231ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 189, 197], OperandSize::Dword)
}

#[test]
fn vfnmadd231ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EAX, ESI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 189, 52, 176], OperandSize::Dword)
}

#[test]
fn vfnmadd231ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 189, 230], OperandSize::Qword)
}

#[test]
fn vfnmadd231ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RBX, RDI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 189, 28, 123], OperandSize::Qword)
}

#[test]
fn vfnmadd231ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 85, 190, 189, 204], OperandSize::Dword)
}

#[test]
fn vfnmadd231ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 101, 142, 189, 49], OperandSize::Dword)
}

#[test]
fn vfnmadd231ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SS, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 61, 186, 189, 207], OperandSize::Qword)
}

#[test]
fn vfnmadd231ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SS, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM31)), operand3: Some(IndirectScaledIndexed(RDX, RBX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 5, 133, 189, 44, 90], OperandSize::Qword)
}

