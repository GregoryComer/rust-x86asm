use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsraw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(66)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 113, 225, 66], OperandSize::Dword)
}

#[test]
fn vpsraw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(106)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 113, 225, 106], OperandSize::Qword)
}

#[test]
fn vpsraw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(109)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 113, 224, 109], OperandSize::Dword)
}

#[test]
fn vpsraw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(102)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 113, 229, 102], OperandSize::Qword)
}

#[test]
fn vpsraw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(109)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 85, 138, 113, 226, 109], OperandSize::Dword)
}

#[test]
fn vpsraw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(34)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 69, 139, 113, 35, 34], OperandSize::Dword)
}

#[test]
fn vpsraw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM14)), operand3: Some(Literal8(9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 209, 21, 133, 113, 230, 9], OperandSize::Qword)
}

#[test]
fn vpsraw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM8)), operand2: Some(IndirectScaledIndexed(RAX, RAX, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(38)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 61, 138, 113, 36, 192, 38], OperandSize::Qword)
}

#[test]
fn vpsraw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Literal8(49)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 174, 113, 225, 49], OperandSize::Dword)
}

#[test]
fn vpsraw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Two, 789651959, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(107)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 101, 175, 113, 164, 81, 247, 33, 17, 47, 107], OperandSize::Dword)
}

#[test]
fn vpsraw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM16)), operand3: Some(Literal8(18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 177, 77, 161, 113, 224, 18], OperandSize::Qword)
}

#[test]
fn vpsraw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM31)), operand2: Some(IndirectScaledIndexed(RDX, RCX, Four, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(110)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 5, 164, 113, 36, 138, 110], OperandSize::Qword)
}

#[test]
fn vpsraw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(Literal8(92)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 101, 202, 113, 224, 92], OperandSize::Dword)
}

#[test]
fn vpsraw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(ZMM1)), operand2: Some(Indirect(ECX, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(51)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 117, 204, 113, 33, 51], OperandSize::Dword)
}

#[test]
fn vpsraw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM19)), operand3: Some(Literal8(68)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 177, 117, 193, 113, 227, 68], OperandSize::Qword)
}

#[test]
fn vpsraw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(ZMM11)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 1119286315, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(100)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 37, 201, 113, 36, 133, 43, 244, 182, 66, 100], OperandSize::Qword)
}

#[test]
fn vpsraw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 225, 207], OperandSize::Dword)
}

#[test]
fn vpsraw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 225, 27], OperandSize::Dword)
}

#[test]
fn vpsraw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 225, 229], OperandSize::Qword)
}

#[test]
fn vpsraw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(RDX, 652204371, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 225, 178, 83, 217, 223, 38], OperandSize::Qword)
}

#[test]
fn vpsraw_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 225, 223], OperandSize::Dword)
}

#[test]
fn vpsraw_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 206608694, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 225, 12, 181, 54, 153, 80, 12], OperandSize::Dword)
}

#[test]
fn vpsraw_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 225, 252], OperandSize::Qword)
}

#[test]
fn vpsraw_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 1575033149, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 225, 52, 157, 61, 25, 225, 93], OperandSize::Qword)
}

#[test]
fn vpsraw_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 69, 137, 225, 207], OperandSize::Dword)
}

#[test]
fn vpsraw_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(ESI, EDX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 117, 139, 225, 52, 150], OperandSize::Dword)
}

#[test]
fn vpsraw_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM22)), operand3: Some(Direct(XMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 177, 77, 129, 225, 251], OperandSize::Qword)
}

#[test]
fn vpsraw_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM16)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 633619606, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 132, 225, 44, 205, 150, 68, 196, 37], OperandSize::Qword)
}

#[test]
fn vpsraw_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 69, 170, 225, 247], OperandSize::Dword)
}

#[test]
fn vpsraw_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 173, 225, 11], OperandSize::Dword)
}

#[test]
fn vpsraw_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 49, 69, 172, 225, 210], OperandSize::Qword)
}

#[test]
fn vpsraw_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM23)), operand3: Some(IndirectScaledIndexed(RSI, RAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 69, 163, 225, 52, 70], OperandSize::Qword)
}

#[test]
fn vpsraw_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 203, 225, 197], OperandSize::Dword)
}

#[test]
fn vpsraw_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 117, 207, 225, 22], OperandSize::Dword)
}

#[test]
fn vpsraw_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM8)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 61, 207, 225, 198], OperandSize::Qword)
}

#[test]
fn vpsraw_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM9)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Four, 309371901, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 53, 202, 225, 172, 179, 253, 163, 112, 18], OperandSize::Qword)
}

