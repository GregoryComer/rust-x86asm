use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpshufhw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(91)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 112, 219, 91], OperandSize::Dword)
}

#[test]
fn vpshufhw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Eight, 1709939988, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(18)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 112, 188, 195, 20, 157, 235, 101, 18], OperandSize::Dword)
}

#[test]
fn vpshufhw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(114)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 112, 216, 114], OperandSize::Qword)
}

#[test]
fn vpshufhw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(RDX, 1310562036, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(13)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 112, 146, 244, 150, 29, 78, 13], OperandSize::Qword)
}

#[test]
fn vpshufhw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Literal8(9)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 112, 242, 9], OperandSize::Dword)
}

#[test]
fn vpshufhw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 1258432092, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(20)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 112, 4, 93, 92, 38, 2, 75, 20], OperandSize::Dword)
}

#[test]
fn vpshufhw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Literal8(10)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 112, 194, 10], OperandSize::Qword)
}

#[test]
fn vpshufhw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 210009050, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(97)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 112, 20, 197, 218, 123, 132, 12, 97], OperandSize::Qword)
}

#[test]
fn vpshufhw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(41)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 126, 137, 112, 220, 41], OperandSize::Dword)
}

#[test]
fn vpshufhw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 1160076648, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(71)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 126, 142, 112, 52, 69, 104, 93, 37, 69, 71], OperandSize::Dword)
}

#[test]
fn vpshufhw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM27)), operand3: Some(Literal8(19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 17, 126, 137, 112, 227, 19], OperandSize::Qword)
}

#[test]
fn vpshufhw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(XMM20)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Eight, 1536113229, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 126, 139, 112, 164, 202, 77, 58, 143, 91, 10], OperandSize::Qword)
}

#[test]
fn vpshufhw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(55)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 126, 175, 112, 253, 55], OperandSize::Dword)
}

#[test]
fn vpshufhw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(YMM2)), operand2: Some(IndirectDisplaced(EBX, 1191728091, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(49)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 126, 175, 112, 147, 219, 83, 8, 71, 49], OperandSize::Dword)
}

#[test]
fn vpshufhw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(53)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 126, 170, 112, 211, 53], OperandSize::Qword)
}

#[test]
fn vpshufhw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(YMM12)), operand2: Some(IndirectScaledDisplaced(RSI, Four, 1682739730, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 126, 170, 112, 36, 181, 18, 146, 76, 100, 27], OperandSize::Qword)
}

#[test]
fn vpshufhw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM5)), operand3: Some(Literal8(51)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 126, 205, 112, 229, 51], OperandSize::Dword)
}

#[test]
fn vpshufhw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledIndexed(ECX, ECX, Eight, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 126, 203, 112, 52, 201, 7], OperandSize::Dword)
}

#[test]
fn vpshufhw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM1)), operand3: Some(Literal8(82)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 126, 205, 112, 233, 82], OperandSize::Qword)
}

#[test]
fn vpshufhw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(ZMM8)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 539877498, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(127)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 126, 203, 112, 4, 149, 122, 224, 45, 32, 127], OperandSize::Qword)
}

