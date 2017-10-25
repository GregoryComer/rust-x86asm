use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsraw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(33)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 113, 225, 33], OperandSize::Dword)
}

#[test]
fn vpsraw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(56)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 113, 229, 56], OperandSize::Qword)
}

#[test]
fn vpsraw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(36)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 113, 229, 36], OperandSize::Dword)
}

#[test]
fn vpsraw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Literal8(117)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 113, 226, 117], OperandSize::Qword)
}

#[test]
fn vpsraw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(47)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 101, 139, 113, 225, 47], OperandSize::Dword)
}

#[test]
fn vpsraw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(75)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 101, 140, 113, 34, 75], OperandSize::Dword)
}

#[test]
fn vpsraw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM9)), operand3: Some(Literal8(97)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 209, 13, 135, 113, 225, 97], OperandSize::Qword)
}

#[test]
fn vpsraw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM22)), operand2: Some(IndirectScaledIndexed(RAX, RDX, Two, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 77, 133, 113, 36, 80, 22], OperandSize::Qword)
}

#[test]
fn vpsraw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(51)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 117, 174, 113, 224, 51], OperandSize::Dword)
}

#[test]
fn vpsraw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexed(EAX, EDI, Eight, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(68)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 77, 169, 113, 36, 248, 68], OperandSize::Dword)
}

#[test]
fn vpsraw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM14)), operand3: Some(Literal8(48)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 209, 69, 173, 113, 230, 48], OperandSize::Qword)
}

#[test]
fn vpsraw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM12)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 731734955, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(94)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 29, 169, 113, 36, 205, 171, 99, 157, 43, 94], OperandSize::Qword)
}

#[test]
fn vpsraw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM4)), operand3: Some(Literal8(120)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 85, 205, 113, 228, 120], OperandSize::Dword)
}

#[test]
fn vpsraw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectDisplaced(ECX, 1097945047, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(112)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 69, 203, 113, 161, 215, 79, 113, 65, 112], OperandSize::Dword)
}

#[test]
fn vpsraw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM19)), operand3: Some(Literal8(126)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 177, 93, 198, 113, 227, 126], OperandSize::Qword)
}

#[test]
fn vpsraw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(ZMM18)), operand2: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 109, 193, 113, 39, 31], OperandSize::Qword)
}

#[test]
fn vpsraw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 225, 216], OperandSize::Dword)
}

#[test]
fn vpsraw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 225, 34], OperandSize::Dword)
}

#[test]
fn vpsraw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 225, 251], OperandSize::Qword)
}

#[test]
fn vpsraw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(RDI, RSI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 225, 36, 119], OperandSize::Qword)
}

#[test]
fn vpsraw_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 225, 238], OperandSize::Dword)
}

#[test]
fn vpsraw_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 225, 23], OperandSize::Dword)
}

#[test]
fn vpsraw_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 225, 211], OperandSize::Qword)
}

#[test]
fn vpsraw_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 340096854, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 225, 28, 117, 86, 119, 69, 20], OperandSize::Qword)
}

#[test]
fn vpsraw_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 109, 141, 225, 227], OperandSize::Dword)
}

#[test]
fn vpsraw_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 1955426874, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 77, 139, 225, 52, 77, 58, 114, 141, 116], OperandSize::Dword)
}

#[test]
fn vpsraw_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM16)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 17, 125, 132, 225, 192], OperandSize::Qword)
}

#[test]
fn vpsraw_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM8)), operand3: Some(IndirectScaledIndexed(RDX, RAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 61, 139, 225, 4, 66], OperandSize::Qword)
}

#[test]
fn vpsraw_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 169, 225, 235], OperandSize::Dword)
}

#[test]
fn vpsraw_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(ESI, EBX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 69, 174, 225, 60, 222], OperandSize::Dword)
}

#[test]
fn vpsraw_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM24)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 61, 161, 225, 203], OperandSize::Qword)
}

#[test]
fn vpsraw_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM23)), operand3: Some(IndirectScaledIndexed(RDX, RSI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 69, 167, 225, 44, 178], OperandSize::Qword)
}

#[test]
fn vpsraw_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 206, 225, 239], OperandSize::Dword)
}

#[test]
fn vpsraw_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(ESI, EDI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 117, 204, 225, 28, 254], OperandSize::Dword)
}

#[test]
fn vpsraw_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM30)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 13, 193, 225, 201], OperandSize::Qword)
}

#[test]
fn vpsraw_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM24)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 1425949224, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 61, 199, 225, 36, 133, 40, 66, 254, 84], OperandSize::Qword)
}

