use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpshufhw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(58)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 112, 203, 58], OperandSize::Dword)
}

#[test]
fn vpshufhw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(EAX, 1691404394, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(115)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 112, 176, 106, 200, 208, 100, 115], OperandSize::Dword)
}

#[test]
fn vpshufhw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 112, 201, 0], OperandSize::Qword)
}

#[test]
fn vpshufhw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(RAX, RBX, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(52)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 112, 52, 216, 52], OperandSize::Qword)
}

#[test]
fn vpshufhw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 112, 220, 5], OperandSize::Dword)
}

#[test]
fn vpshufhw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Four, 792063783, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(124)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 112, 132, 176, 39, 239, 53, 47, 124], OperandSize::Dword)
}

#[test]
fn vpshufhw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(100)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 112, 197, 100], OperandSize::Qword)
}

#[test]
fn vpshufhw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 1728477699, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(88)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 112, 60, 189, 3, 122, 6, 103, 88], OperandSize::Qword)
}

#[test]
fn vpshufhw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(41)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 126, 140, 112, 199, 41], OperandSize::Dword)
}

#[test]
fn vpshufhw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 1155832723, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(96)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 126, 140, 112, 20, 85, 147, 155, 228, 68, 96], OperandSize::Dword)
}

#[test]
fn vpshufhw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM28)), operand3: Some(Literal8(26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 17, 126, 138, 112, 236, 26], OperandSize::Qword)
}

#[test]
fn vpshufhw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(XMM18)), operand2: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(113)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 126, 137, 112, 19, 113], OperandSize::Qword)
}

#[test]
fn vpshufhw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(107)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 126, 171, 112, 236, 107], OperandSize::Dword)
}

#[test]
fn vpshufhw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 1746489891, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(122)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 126, 173, 112, 36, 205, 35, 82, 25, 104, 122], OperandSize::Dword)
}

#[test]
fn vpshufhw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM27)), operand3: Some(Literal8(38)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 145, 126, 175, 112, 251, 38], OperandSize::Qword)
}

#[test]
fn vpshufhw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(YMM16)), operand2: Some(IndirectScaledIndexed(RDI, RSI, Four, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(77)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 126, 174, 112, 4, 183, 77], OperandSize::Qword)
}

#[test]
fn vpshufhw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM7)), operand3: Some(Literal8(9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 126, 202, 112, 207, 9], OperandSize::Dword)
}

#[test]
fn vpshufhw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(ZMM3)), operand2: Some(Indirect(EDI, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(43)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 126, 207, 112, 31, 43], OperandSize::Dword)
}

#[test]
fn vpshufhw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM21)), operand3: Some(Literal8(12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 49, 126, 201, 112, 237, 12], OperandSize::Qword)
}

#[test]
fn vpshufhw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFHW, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 809626498, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(43)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 126, 206, 112, 60, 197, 130, 235, 65, 48, 43], OperandSize::Qword)
}

