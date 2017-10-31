use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vdbpsadbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: Some(Literal8(83)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 69, 142, 66, 245, 83], OperandSize::Dword)
}

#[test]
fn vdbpsadbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(112)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 125, 140, 66, 14, 112], OperandSize::Dword)
}

#[test]
fn vdbpsadbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM16)), operand4: Some(Literal8(18)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 35, 29, 142, 66, 208, 18], OperandSize::Qword)
}

#[test]
fn vdbpsadbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM13)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(13)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 21, 138, 66, 51, 13], OperandSize::Qword)
}

#[test]
fn vdbpsadbw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM7)), operand4: Some(Literal8(36)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 85, 174, 66, 239, 36], OperandSize::Dword)
}

#[test]
fn vdbpsadbw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(43)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 117, 169, 66, 34, 43], OperandSize::Dword)
}

#[test]
fn vdbpsadbw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM30)), operand3: Some(Direct(YMM26)), operand4: Some(Literal8(95)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 131, 13, 165, 66, 194, 95], OperandSize::Qword)
}

#[test]
fn vdbpsadbw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Two, 1310088776, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(88)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 93, 172, 66, 148, 120, 72, 94, 22, 78, 88], OperandSize::Qword)
}

#[test]
fn vdbpsadbw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM3)), operand4: Some(Literal8(35)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 85, 201, 66, 211, 35], OperandSize::Dword)
}

#[test]
fn vdbpsadbw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: Some(Indirect(EAX, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(13)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 93, 201, 66, 24, 13], OperandSize::Dword)
}

#[test]
fn vdbpsadbw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM13)), operand3: Some(Direct(ZMM30)), operand4: Some(Literal8(43)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 131, 21, 203, 66, 222, 43], OperandSize::Qword)
}

#[test]
fn vdbpsadbw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM8)), operand3: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(120)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 61, 203, 66, 58, 120], OperandSize::Qword)
}

