use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vreducess_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCESS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: Some(Literal8(7)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 85, 153, 87, 207, 7], OperandSize::Dword)
}

#[test]
fn vreducess_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCESS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(ESI, EDI, Eight, Some(OperandSize::Dword), None)), operand4: Some(Literal8(20)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 109, 141, 87, 60, 254, 20], OperandSize::Dword)
}

#[test]
fn vreducess_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCESS, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM17)), operand4: Some(Literal8(98)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 35, 85, 151, 87, 209, 98], OperandSize::Qword)
}

#[test]
fn vreducess_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCESS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 217707841, Some(OperandSize::Dword), None)), operand4: Some(Literal8(41)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 101, 133, 87, 44, 117, 65, 245, 249, 12, 41], OperandSize::Qword)
}

