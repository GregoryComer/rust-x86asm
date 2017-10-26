use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpshufhw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(59)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 112, 245, 59], OperandSize::Dword)
}

#[test]
fn vpshufhw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Two, 1704356097, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 112, 148, 81, 1, 105, 150, 101, 0], OperandSize::Dword)
}

#[test]
fn vpshufhw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(122)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 112, 236, 122], OperandSize::Qword)
}

#[test]
fn vpshufhw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(15)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 112, 25, 15], OperandSize::Qword)
}

#[test]
fn vpshufhw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Literal8(46)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 112, 201, 46], OperandSize::Dword)
}

#[test]
fn vpshufhw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(YMM7)), operand2: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(47)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 112, 56, 47], OperandSize::Dword)
}

#[test]
fn vpshufhw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(43)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 112, 237, 43], OperandSize::Qword)
}

#[test]
fn vpshufhw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Eight, 692550794, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(107)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 112, 148, 203, 138, 124, 71, 41, 107], OperandSize::Qword)
}

#[test]
fn vpshufhw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(78)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 126, 142, 112, 208, 78], OperandSize::Dword)
}

#[test]
fn vpshufhw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Eight, 1917789772, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(50)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 126, 141, 112, 148, 243, 76, 38, 79, 114, 50], OperandSize::Dword)
}

#[test]
fn vpshufhw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(102)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 126, 138, 112, 216, 102], OperandSize::Qword)
}

#[test]
fn vpshufhw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(RBX, RBX, Four, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(37)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 126, 143, 112, 28, 155, 37], OperandSize::Qword)
}

#[test]
fn vpshufhw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Literal8(73)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 126, 170, 112, 202, 73], OperandSize::Dword)
}

#[test]
fn vpshufhw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Eight, 1694190297, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(43)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 126, 172, 112, 156, 246, 217, 74, 251, 100, 43], OperandSize::Dword)
}

#[test]
fn vpshufhw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 126, 169, 112, 231, 30], OperandSize::Qword)
}

#[test]
fn vpshufhw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(YMM8)), operand2: Some(IndirectScaledIndexed(RBX, RCX, Eight, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(53)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 126, 170, 112, 4, 203, 53], OperandSize::Qword)
}

#[test]
fn vpshufhw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(Literal8(116)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 126, 207, 112, 216, 116], OperandSize::Dword)
}

#[test]
fn vpshufhw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Two, 1705298004, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(63)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 126, 205, 112, 156, 75, 84, 200, 164, 101, 63], OperandSize::Dword)
}

#[test]
fn vpshufhw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM16)), operand3: Some(Literal8(98)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 161, 126, 204, 112, 192, 98], OperandSize::Qword)
}

#[test]
fn vpshufhw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(ZMM20)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Four, 1522917430, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(42)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 126, 202, 112, 164, 185, 54, 224, 197, 90, 42], OperandSize::Qword)
}

