use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpshuflw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(8)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 112, 219, 8], OperandSize::Dword)
}

#[test]
fn vpshuflw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(105)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 112, 7, 105], OperandSize::Dword)
}

#[test]
fn vpshuflw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(54)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 112, 227, 54], OperandSize::Qword)
}

#[test]
fn vpshuflw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(RSI, RCX, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(51)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 112, 52, 206, 51], OperandSize::Qword)
}

#[test]
fn vpshuflw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(29)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 112, 199, 29], OperandSize::Dword)
}

#[test]
fn vpshuflw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(YMM7)), operand2: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(87)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 112, 57, 87], OperandSize::Dword)
}

#[test]
fn vpshuflw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Literal8(80)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 112, 225, 80], OperandSize::Qword)
}

#[test]
fn vpshuflw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 287569098, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(9)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 112, 4, 133, 202, 244, 35, 17, 9], OperandSize::Qword)
}

#[test]
fn vpshuflw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 127, 141, 112, 236, 9], OperandSize::Dword)
}

#[test]
fn vpshuflw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(EDI, ECX, Two, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 127, 138, 112, 60, 79, 12], OperandSize::Dword)
}

#[test]
fn vpshuflw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(90)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 127, 141, 112, 225, 90], OperandSize::Qword)
}

#[test]
fn vpshuflw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(RCX, RAX, Two, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(61)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 127, 142, 112, 12, 65, 61], OperandSize::Qword)
}

#[test]
fn vpshuflw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Literal8(19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 127, 171, 112, 242, 19], OperandSize::Dword)
}

#[test]
fn vpshuflw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 956090828, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(121)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 127, 171, 112, 60, 133, 204, 201, 252, 56, 121], OperandSize::Dword)
}

#[test]
fn vpshuflw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM14)), operand3: Some(Literal8(108)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 81, 127, 175, 112, 206, 108], OperandSize::Qword)
}

#[test]
fn vpshuflw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(YMM29)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Eight, 1468680855, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(108)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 127, 172, 112, 172, 240, 151, 74, 138, 87, 108], OperandSize::Qword)
}

#[test]
fn vpshuflw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM5)), operand3: Some(Literal8(28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 127, 205, 112, 229, 28], OperandSize::Dword)
}

#[test]
fn vpshuflw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectDisplaced(EDX, 32531372, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 127, 202, 112, 178, 172, 99, 240, 1, 19], OperandSize::Dword)
}

#[test]
fn vpshuflw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM14)), operand3: Some(Literal8(111)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 209, 127, 203, 112, 238, 111], OperandSize::Qword)
}

#[test]
fn vpshuflw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(ZMM23)), operand2: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(98)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 127, 203, 112, 58, 98], OperandSize::Qword)
}

