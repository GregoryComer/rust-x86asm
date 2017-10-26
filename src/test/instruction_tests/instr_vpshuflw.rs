use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpshuflw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(79)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 112, 194, 79], OperandSize::Dword)
}

#[test]
fn vpshuflw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 1880263687, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(85)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 112, 52, 253, 7, 140, 18, 112, 85], OperandSize::Dword)
}

#[test]
fn vpshuflw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(81)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 112, 200, 81], OperandSize::Qword)
}

#[test]
fn vpshuflw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 556101297, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(41)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 112, 20, 245, 177, 110, 37, 33, 41], OperandSize::Qword)
}

#[test]
fn vpshuflw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(58)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 112, 207, 58], OperandSize::Dword)
}

#[test]
fn vpshuflw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(YMM7)), operand2: Some(IndirectDisplaced(EAX, 751782336, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(43)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 112, 184, 192, 73, 207, 44, 43], OperandSize::Dword)
}

#[test]
fn vpshuflw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(30)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 112, 216, 30], OperandSize::Qword)
}

#[test]
fn vpshuflw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexed(RCX, RCX, Two, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(105)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 112, 52, 73, 105], OperandSize::Qword)
}

#[test]
fn vpshuflw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(109)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 127, 137, 112, 195, 109], OperandSize::Dword)
}

#[test]
fn vpshuflw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(EBX, ESI, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(106)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 127, 141, 112, 52, 243, 106], OperandSize::Dword)
}

#[test]
fn vpshuflw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM16)), operand3: Some(Literal8(47)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 177, 127, 142, 112, 216, 47], OperandSize::Qword)
}

#[test]
fn vpshuflw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(XMM23)), operand2: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(86)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 127, 141, 112, 58, 86], OperandSize::Qword)
}

#[test]
fn vpshuflw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(40)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 127, 171, 112, 245, 40], OperandSize::Dword)
}

#[test]
fn vpshuflw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 454130494, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(75)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 127, 175, 112, 36, 141, 62, 123, 17, 27, 75], OperandSize::Dword)
}

#[test]
fn vpshuflw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM20)), operand3: Some(Literal8(41)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 161, 127, 171, 112, 220, 41], OperandSize::Qword)
}

#[test]
fn vpshuflw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(YMM16)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 40256208, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(96)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 127, 171, 112, 4, 149, 208, 66, 102, 2, 96], OperandSize::Qword)
}

#[test]
fn vpshuflw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM6)), operand3: Some(Literal8(95)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 127, 204, 112, 198, 95], OperandSize::Dword)
}

#[test]
fn vpshuflw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(ZMM4)), operand2: Some(Indirect(ESI, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(121)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 127, 204, 112, 38, 121], OperandSize::Dword)
}

#[test]
fn vpshuflw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM15)), operand3: Some(Literal8(122)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 193, 127, 201, 112, 215, 122], OperandSize::Qword)
}

#[test]
fn vpshuflw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(ZMM27)), operand2: Some(Indirect(RSI, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 127, 201, 112, 30, 11], OperandSize::Qword)
}

