use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpshuflw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(74)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 112, 201, 74], OperandSize::Dword)
}

#[test]
fn vpshuflw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Eight, 1759714037, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(20)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 112, 132, 215, 245, 26, 227, 104, 20], OperandSize::Dword)
}

#[test]
fn vpshuflw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(78)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 112, 196, 78], OperandSize::Qword)
}

#[test]
fn vpshuflw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 297931393, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(114)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 112, 44, 253, 129, 18, 194, 17, 114], OperandSize::Qword)
}

#[test]
fn vpshuflw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(36)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 112, 227, 36], OperandSize::Dword)
}

#[test]
fn vpshuflw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 1571837573, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(28)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 112, 60, 133, 133, 86, 176, 93, 28], OperandSize::Dword)
}

#[test]
fn vpshuflw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(16)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 112, 199, 16], OperandSize::Qword)
}

#[test]
fn vpshuflw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 942238549, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(20)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 112, 28, 245, 85, 107, 41, 56, 20], OperandSize::Qword)
}

#[test]
fn vpshuflw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(41)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 127, 140, 112, 209, 41], OperandSize::Dword)
}

#[test]
fn vpshuflw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(ESI, 383825137, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(94)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 127, 142, 112, 166, 241, 180, 224, 22, 94], OperandSize::Dword)
}

#[test]
fn vpshuflw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM20)), operand3: Some(Literal8(43)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 161, 127, 143, 112, 196, 43], OperandSize::Qword)
}

#[test]
fn vpshuflw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(XMM8)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 2082475250, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(102)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 127, 142, 112, 4, 85, 242, 12, 32, 124, 102], OperandSize::Qword)
}

#[test]
fn vpshuflw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(87)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 127, 173, 112, 204, 87], OperandSize::Dword)
}

#[test]
fn vpshuflw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(YMM0)), operand2: Some(IndirectDisplaced(ESI, 197017719, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(98)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 127, 174, 112, 134, 119, 64, 190, 11, 98], OperandSize::Dword)
}

#[test]
fn vpshuflw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM17)), operand3: Some(Literal8(17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 177, 127, 171, 112, 209, 17], OperandSize::Qword)
}

#[test]
fn vpshuflw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(YMM26)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 1571093539, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(75)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 127, 171, 112, 20, 253, 35, 252, 164, 93, 75], OperandSize::Qword)
}

#[test]
fn vpshuflw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM1)), operand3: Some(Literal8(117)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 127, 202, 112, 249, 117], OperandSize::Dword)
}

#[test]
fn vpshuflw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Four, 985661315, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(74)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 127, 202, 112, 188, 186, 131, 255, 191, 58, 74], OperandSize::Dword)
}

#[test]
fn vpshuflw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM19)), operand3: Some(Literal8(95)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 177, 127, 201, 112, 219, 95], OperandSize::Qword)
}

#[test]
fn vpshuflw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(ZMM26)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 332994980, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(46)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 127, 202, 112, 20, 245, 164, 25, 217, 19, 46], OperandSize::Qword)
}

