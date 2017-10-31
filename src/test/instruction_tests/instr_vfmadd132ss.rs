use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmadd132ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 153, 241], OperandSize::Dword)
}

#[test]
fn vfmadd132ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 352493850, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 153, 12, 149, 26, 161, 2, 21], OperandSize::Dword)
}

#[test]
fn vfmadd132ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 153, 196], OperandSize::Qword)
}

#[test]
fn vfmadd132ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(RCX, 1655863941, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 153, 137, 133, 122, 178, 98], OperandSize::Qword)
}

#[test]
fn vfmadd132ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 77, 156, 153, 200], OperandSize::Dword)
}

#[test]
fn vfmadd132ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(ECX, EDI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 153, 60, 185], OperandSize::Dword)
}

#[test]
fn vfmadd132ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM20)), operand3: Some(Direct(XMM19)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 178, 93, 242, 153, 243], OperandSize::Qword)
}

#[test]
fn vfmadd132ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132SS, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 93, 143, 153, 11], OperandSize::Qword)
}

