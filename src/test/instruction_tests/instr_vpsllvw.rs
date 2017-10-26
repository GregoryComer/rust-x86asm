use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsllvw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 205, 142, 18, 252], OperandSize::Dword)
}

#[test]
fn vpsllvw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 1625304224, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 205, 143, 18, 36, 181, 160, 44, 224, 96], OperandSize::Dword)
}

#[test]
fn vpsllvw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 194, 237, 139, 18, 226], OperandSize::Qword)
}

#[test]
fn vpsllvw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Eight, 258369662, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 133, 138, 18, 140, 207, 126, 104, 102, 15], OperandSize::Qword)
}

#[test]
fn vpsllvw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 229, 169, 18, 220], OperandSize::Dword)
}

#[test]
fn vpsllvw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 229, 172, 18, 17], OperandSize::Dword)
}

#[test]
fn vpsllvw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM12)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 157, 171, 18, 245], OperandSize::Qword)
}

#[test]
fn vpsllvw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM31)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Four, 1417201469, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 133, 164, 18, 164, 128, 61, 199, 120, 84], OperandSize::Qword)
}

#[test]
fn vpsllvw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 213, 201, 18, 210], OperandSize::Dword)
}

#[test]
fn vpsllvw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(EDX, 459675828, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 237, 203, 18, 178, 180, 24, 102, 27], OperandSize::Dword)
}

#[test]
fn vpsllvw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM8)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 189, 201, 18, 221], OperandSize::Qword)
}

#[test]
fn vpsllvw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM29)), operand3: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 149, 197, 18, 0], OperandSize::Qword)
}

