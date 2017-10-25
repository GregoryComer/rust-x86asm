use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vcmpss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(127)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 194, 236, 127], OperandSize::Dword)
}

fn vcmpss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EAX, 1797576737, Some(OperandSize::Dword), None)), operand4: Some(Literal8(15)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 210, 194, 160, 33, 216, 36, 107, 15], OperandSize::Dword)
}

fn vcmpss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(68)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 242, 194, 250, 68], OperandSize::Qword)
}

fn vcmpss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(RDX, RAX, Two, Some(OperandSize::Dword), None)), operand4: Some(Literal8(44)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 202, 194, 4, 66, 44], OperandSize::Qword)
}

fn vcmpss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSS, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(119)), lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 110, 27, 194, 230, 119], OperandSize::Dword)
}

fn vcmpss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSS, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Two, Some(OperandSize::Dword), None)), operand4: Some(Literal8(89)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 126, 14, 194, 12, 65, 89], OperandSize::Dword)
}

fn vcmpss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSS, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM31)), operand4: Some(Literal8(82)), lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 145, 86, 31, 194, 255, 82], OperandSize::Qword)
}

fn vcmpss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSS, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Eight, Some(OperandSize::Dword), None)), operand4: Some(Literal8(120)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 54, 4, 194, 20, 246, 120], OperandSize::Qword)
}

