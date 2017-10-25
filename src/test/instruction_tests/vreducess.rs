use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vreducess_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCESS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM1)), operand4: Some(Literal8(41)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 77, 153, 87, 249, 41], OperandSize::Dword)
}

fn vreducess_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCESS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(EBX, 1490142827, Some(OperandSize::Dword), None)), operand4: Some(Literal8(83)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 125, 138, 87, 171, 107, 198, 209, 88, 83], OperandSize::Dword)
}

fn vreducess_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCESS, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM13)), operand3: Some(Direct(XMM12)), operand4: Some(Literal8(38)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 83, 21, 156, 87, 220, 38], OperandSize::Qword)
}

fn vreducess_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCESS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(124)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 227, 101, 139, 87, 0, 124], OperandSize::Qword)
}

