use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpsraq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(68)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 213, 137, 114, 229, 68], OperandSize::Dword)
}

fn vpsraq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 44839510, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(123)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 205, 140, 114, 36, 253, 86, 50, 172, 2, 123], OperandSize::Dword)
}

fn vpsraq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand3: Some(Literal8(76)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 237, 153, 114, 35, 76], OperandSize::Dword)
}

fn vpsraq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(110)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 213, 142, 114, 230, 110], OperandSize::Qword)
}

fn vpsraq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(XMM28)), operand2: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(95)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 157, 132, 114, 35, 95], OperandSize::Qword)
}

fn vpsraq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RDX, RDI, Eight, Some(OperandSize::Qword), None)), operand3: Some(Literal8(38)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 213, 156, 114, 36, 250, 38], OperandSize::Qword)
}

fn vpsraq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(111)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 205, 175, 114, 230, 111], OperandSize::Dword)
}

fn vpsraq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(YMM5)), operand2: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(123)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 213, 173, 114, 38, 123], OperandSize::Dword)
}

fn vpsraq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Two, 703845237, Some(OperandSize::Qword), None)), operand3: Some(Literal8(121)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 213, 186, 114, 164, 121, 117, 211, 243, 41, 121], OperandSize::Dword)
}

fn vpsraq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM29)), operand3: Some(Literal8(23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 145, 181, 162, 114, 229, 23], OperandSize::Qword)
}

fn vpsraq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(YMM17)), operand2: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(127)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 245, 161, 114, 34, 127], OperandSize::Qword)
}

fn vpsraq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(YMM21)), operand2: Some(IndirectScaledIndexed(RSI, RCX, Eight, Some(OperandSize::Qword), None)), operand3: Some(Literal8(56)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 213, 179, 114, 36, 206, 56], OperandSize::Qword)
}

fn vpsraq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM3)), operand3: Some(Literal8(2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 213, 207, 114, 227, 2], OperandSize::Dword)
}

fn vpsraq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 543113994, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(104)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 213, 204, 114, 36, 141, 10, 67, 95, 32, 104], OperandSize::Dword)
}

fn vpsraq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledDisplaced(EDX, Eight, 64991220, Some(OperandSize::Qword), None)), operand3: Some(Literal8(127)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 229, 217, 114, 36, 213, 244, 175, 223, 3, 127], OperandSize::Dword)
}

fn vpsraq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM28)), operand3: Some(Literal8(23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 145, 221, 195, 114, 228, 23], OperandSize::Qword)
}

fn vpsraq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(ZMM8)), operand2: Some(IndirectScaledIndexed(RDX, RSI, Two, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 189, 201, 114, 36, 114, 22], OperandSize::Qword)
}

fn vpsraq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(ZMM12)), operand2: Some(IndirectDisplaced(RAX, 838481442, Some(OperandSize::Qword), None)), operand3: Some(Literal8(60)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 157, 219, 114, 160, 34, 54, 250, 49, 60], OperandSize::Qword)
}

fn vpsraq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 197, 137, 226, 202], OperandSize::Dword)
}

fn vpsraq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 237, 138, 226, 8], OperandSize::Dword)
}

fn vpsraq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM9)), operand3: Some(Direct(XMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 65, 181, 142, 226, 209], OperandSize::Qword)
}

fn vpsraq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectDisplaced(RDI, 2028943349, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 173, 134, 226, 167, 245, 55, 239, 120], OperandSize::Qword)
}

fn vpsraq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 221, 173, 226, 232], OperandSize::Dword)
}

fn vpsraq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 199615451, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 174, 226, 12, 157, 219, 227, 229, 11], OperandSize::Dword)
}

fn vpsraq_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM22)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 161, 205, 166, 226, 250], OperandSize::Qword)
}

fn vpsraq_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 221, 174, 226, 43], OperandSize::Qword)
}

fn vpsraq_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 197, 203, 226, 230], OperandSize::Dword)
}

fn vpsraq_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 1074569406, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 197, 205, 226, 28, 141, 190, 160, 12, 64], OperandSize::Dword)
}

fn vpsraq_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM25)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 145, 181, 198, 226, 208], OperandSize::Qword)
}

fn vpsraq_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM23)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 197, 198, 226, 8], OperandSize::Qword)
}

