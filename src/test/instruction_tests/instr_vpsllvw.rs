use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsllvw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 245, 137, 18, 210], OperandSize::Dword)
}

#[test]
fn vpsllvw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EDX, 277496328, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 221, 139, 18, 130, 8, 66, 138, 16], OperandSize::Dword)
}

#[test]
fn vpsllvw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 146, 157, 134, 18, 204], OperandSize::Qword)
}

#[test]
fn vpsllvw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM16)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Four, 363787369, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 253, 134, 18, 132, 143, 105, 244, 174, 21], OperandSize::Qword)
}

#[test]
fn vpsllvw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 205, 175, 18, 231], OperandSize::Dword)
}

#[test]
fn vpsllvw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(EDI, 323988328, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 205, 174, 18, 159, 104, 171, 79, 19], OperandSize::Dword)
}

#[test]
fn vpsllvw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM24)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 189, 162, 18, 202], OperandSize::Qword)
}

#[test]
fn vpsllvw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM23)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Four, 2046325663, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 197, 167, 18, 164, 143, 159, 115, 248, 121], OperandSize::Qword)
}

#[test]
fn vpsllvw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 213, 202, 18, 246], OperandSize::Dword)
}

#[test]
fn vpsllvw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(EDX, EDX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 221, 201, 18, 28, 146], OperandSize::Dword)
}

#[test]
fn vpsllvw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM16)), operand3: Some(Direct(ZMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 82, 253, 195, 18, 214], OperandSize::Qword)
}

#[test]
fn vpsllvw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM29)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 149, 199, 18, 4, 115], OperandSize::Qword)
}

