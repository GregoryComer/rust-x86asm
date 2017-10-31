use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsllvw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 205, 139, 18, 196], OperandSize::Dword)
}

#[test]
fn vpsllvw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EBX, 2013440414, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 205, 138, 18, 187, 158, 169, 2, 120], OperandSize::Dword)
}

#[test]
fn vpsllvw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM14)), operand3: Some(Direct(XMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 66, 141, 142, 18, 241], OperandSize::Qword)
}

#[test]
fn vpsllvw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 173, 140, 18, 36, 118], OperandSize::Qword)
}

#[test]
fn vpsllvw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 237, 169, 18, 255], OperandSize::Dword)
}

#[test]
fn vpsllvw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 245, 172, 18, 6], OperandSize::Dword)
}

#[test]
fn vpsllvw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 34, 213, 171, 18, 255], OperandSize::Qword)
}

#[test]
fn vpsllvw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM18)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 237, 167, 18, 42], OperandSize::Qword)
}

#[test]
fn vpsllvw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 197, 206, 18, 210], OperandSize::Dword)
}

#[test]
fn vpsllvw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Two, 997456516, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 245, 207, 18, 140, 73, 132, 250, 115, 59], OperandSize::Dword)
}

#[test]
fn vpsllvw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 2, 253, 207, 18, 239], OperandSize::Qword)
}

#[test]
fn vpsllvw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM17)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Four, 89181115, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 245, 198, 18, 180, 150, 187, 203, 80, 5], OperandSize::Qword)
}

